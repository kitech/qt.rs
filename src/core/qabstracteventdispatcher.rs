// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qabstracteventdispatcher.h
// dst-file: /src/core/qabstracteventdispatcher.rs
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
use super::qthread::QThread; // 773
use super::qbytearray::QByteArray; // 773
use super::qsocketnotifier::QSocketNotifier; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  bool QAbstractEventDispatcher::hasPendingEvents();
  fn _ZN24QAbstractEventDispatcher16hasPendingEventsEv(qthis: *mut c_void) -> c_char;
  // proto:  void QAbstractEventDispatcher::QAbstractEventDispatcher(QObject * parent);
  fn _ZN24QAbstractEventDispatcherC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QAbstractEventDispatcher * QAbstractEventDispatcher::instance(QThread * thread);
  fn _ZN24QAbstractEventDispatcher8instanceEP7QThread(arg0: *mut c_void);
  // proto:  bool QAbstractEventDispatcher::filterNativeEvent(const QByteArray & eventType, void * message, long * result);
  fn _ZN24QAbstractEventDispatcher17filterNativeEventERK10QByteArrayPvPl(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_long) -> c_char;
  // proto:  void QAbstractEventDispatcher::~QAbstractEventDispatcher();
  fn _ZN24QAbstractEventDispatcherD0Ev(qthis: *mut c_void);
  // proto:  void QAbstractEventDispatcher::aboutToBlock();
  fn _ZN24QAbstractEventDispatcher12aboutToBlockEv(qthis: *mut c_void);
  // proto:  void QAbstractEventDispatcher::interrupt();
  fn _ZN24QAbstractEventDispatcher9interruptEv(qthis: *mut c_void);
  // proto:  void QAbstractEventDispatcher::awake();
  fn _ZN24QAbstractEventDispatcher5awakeEv(qthis: *mut c_void);
  // proto:  void QAbstractEventDispatcher::registerSocketNotifier(QSocketNotifier * notifier);
  fn _ZN24QAbstractEventDispatcher22registerSocketNotifierEP15QSocketNotifier(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAbstractEventDispatcher::flush();
  fn _ZN24QAbstractEventDispatcher5flushEv(qthis: *mut c_void);
  // proto:  void QAbstractEventDispatcher::unregisterSocketNotifier(QSocketNotifier * notifier);
  fn _ZN24QAbstractEventDispatcher24unregisterSocketNotifierEP15QSocketNotifier(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAbstractEventDispatcher::wakeUp();
  fn _ZN24QAbstractEventDispatcher6wakeUpEv(qthis: *mut c_void);
  // proto:  const QMetaObject * QAbstractEventDispatcher::metaObject();
  fn _ZNK24QAbstractEventDispatcher10metaObjectEv(qthis: *mut c_void);
  // proto:  bool QAbstractEventDispatcher::unregisterTimers(QObject * object);
  fn _ZN24QAbstractEventDispatcher16unregisterTimersEP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  int QAbstractEventDispatcher::remainingTime(int timerId);
  fn _ZN24QAbstractEventDispatcher13remainingTimeEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QAbstractEventDispatcher::startingUp();
  fn _ZN24QAbstractEventDispatcher10startingUpEv(qthis: *mut c_void);
  // proto:  void QAbstractEventDispatcher::closingDown();
  fn _ZN24QAbstractEventDispatcher11closingDownEv(qthis: *mut c_void);
  // proto:  bool QAbstractEventDispatcher::unregisterTimer(int timerId);
  fn _ZN24QAbstractEventDispatcher15unregisterTimerEi(qthis: *mut c_void, arg0: c_int) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractEventDispatcher)=1
pub struct QAbstractEventDispatcher {
  pub qclsinst: *mut c_void,
}

  // proto:  bool QAbstractEventDispatcher::hasPendingEvents();
impl /*struct*/ QAbstractEventDispatcher {
  pub fn hasPendingEvents<RetType, T: QAbstractEventDispatcher_hasPendingEvents<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasPendingEvents(self);
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_hasPendingEvents<RetType> {
  fn hasPendingEvents(self , rsthis: &mut QAbstractEventDispatcher) -> RetType;
}

  // proto:  bool QAbstractEventDispatcher::hasPendingEvents();
impl<'a> /*trait*/ QAbstractEventDispatcher_hasPendingEvents<i8> for () {
  fn hasPendingEvents(self , rsthis: &mut QAbstractEventDispatcher) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractEventDispatcher16hasPendingEventsEv()};
    let mut ret = unsafe {_ZN24QAbstractEventDispatcher16hasPendingEventsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractEventDispatcher::QAbstractEventDispatcher(QObject * parent);
impl /*struct*/ QAbstractEventDispatcher {
  pub fn NewQAbstractEventDispatcher<T: QAbstractEventDispatcher_NewQAbstractEventDispatcher>(value: T) -> QAbstractEventDispatcher {
    let rsthis = value.NewQAbstractEventDispatcher();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_NewQAbstractEventDispatcher {
  fn NewQAbstractEventDispatcher(self) -> QAbstractEventDispatcher;
}

  // proto:  void QAbstractEventDispatcher::QAbstractEventDispatcher(QObject * parent);
impl<'a> /*trait*/ QAbstractEventDispatcher_NewQAbstractEventDispatcher for (QObject) {
  fn NewQAbstractEventDispatcher(self) -> QAbstractEventDispatcher {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractEventDispatcherC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QAbstractEventDispatcherC1EP7QObject(qthis, arg0)};
    let rsthis = QAbstractEventDispatcher{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QAbstractEventDispatcher * QAbstractEventDispatcher::instance(QThread * thread);
impl /*struct*/ QAbstractEventDispatcher {
  pub fn instance_s<RetType, T: QAbstractEventDispatcher_instance_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.instance_s();
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_instance_s<RetType> {
  fn instance_s(self ) -> RetType;
}

  // proto: static QAbstractEventDispatcher * QAbstractEventDispatcher::instance(QThread * thread);
impl<'a> /*trait*/ QAbstractEventDispatcher_instance_s<()> for (QThread) {
  fn instance_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractEventDispatcher8instanceEP7QThread()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QAbstractEventDispatcher8instanceEP7QThread(arg0)};
    // return 1;
  }
}

  // proto:  bool QAbstractEventDispatcher::filterNativeEvent(const QByteArray & eventType, void * message, long * result);
impl /*struct*/ QAbstractEventDispatcher {
  pub fn filterNativeEvent<RetType, T: QAbstractEventDispatcher_filterNativeEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.filterNativeEvent(self);
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_filterNativeEvent<RetType> {
  fn filterNativeEvent(self , rsthis: &mut QAbstractEventDispatcher) -> RetType;
}

  // proto:  bool QAbstractEventDispatcher::filterNativeEvent(const QByteArray & eventType, void * message, long * result);
impl<'a> /*trait*/ QAbstractEventDispatcher_filterNativeEvent<i8> for (QByteArray, *mut c_void, &'a mut Vec<i64>) {
  fn filterNativeEvent(self , rsthis: &mut QAbstractEventDispatcher) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractEventDispatcher17filterNativeEventERK10QByteArrayPvPl()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as *mut c_void;
    let arg2 = self.2.as_ptr()  as *mut c_long;
    let mut ret = unsafe {_ZN24QAbstractEventDispatcher17filterNativeEventERK10QByteArrayPvPl(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractEventDispatcher::~QAbstractEventDispatcher();
impl /*struct*/ QAbstractEventDispatcher {
  pub fn FreeQAbstractEventDispatcher<RetType, T: QAbstractEventDispatcher_FreeQAbstractEventDispatcher<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAbstractEventDispatcher(self);
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_FreeQAbstractEventDispatcher<RetType> {
  fn FreeQAbstractEventDispatcher(self , rsthis: &mut QAbstractEventDispatcher) -> RetType;
}

  // proto:  void QAbstractEventDispatcher::~QAbstractEventDispatcher();
impl<'a> /*trait*/ QAbstractEventDispatcher_FreeQAbstractEventDispatcher<()> for () {
  fn FreeQAbstractEventDispatcher(self , rsthis: &mut QAbstractEventDispatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractEventDispatcherD0Ev()};
     unsafe {_ZN24QAbstractEventDispatcherD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractEventDispatcher::aboutToBlock();
impl /*struct*/ QAbstractEventDispatcher {
  pub fn aboutToBlock<RetType, T: QAbstractEventDispatcher_aboutToBlock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.aboutToBlock(self);
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_aboutToBlock<RetType> {
  fn aboutToBlock(self , rsthis: &mut QAbstractEventDispatcher) -> RetType;
}

  // proto:  void QAbstractEventDispatcher::aboutToBlock();
impl<'a> /*trait*/ QAbstractEventDispatcher_aboutToBlock<()> for () {
  fn aboutToBlock(self , rsthis: &mut QAbstractEventDispatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractEventDispatcher12aboutToBlockEv()};
     unsafe {_ZN24QAbstractEventDispatcher12aboutToBlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractEventDispatcher::interrupt();
impl /*struct*/ QAbstractEventDispatcher {
  pub fn interrupt<RetType, T: QAbstractEventDispatcher_interrupt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.interrupt(self);
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_interrupt<RetType> {
  fn interrupt(self , rsthis: &mut QAbstractEventDispatcher) -> RetType;
}

  // proto:  void QAbstractEventDispatcher::interrupt();
impl<'a> /*trait*/ QAbstractEventDispatcher_interrupt<()> for () {
  fn interrupt(self , rsthis: &mut QAbstractEventDispatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractEventDispatcher9interruptEv()};
     unsafe {_ZN24QAbstractEventDispatcher9interruptEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractEventDispatcher::awake();
impl /*struct*/ QAbstractEventDispatcher {
  pub fn awake<RetType, T: QAbstractEventDispatcher_awake<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.awake(self);
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_awake<RetType> {
  fn awake(self , rsthis: &mut QAbstractEventDispatcher) -> RetType;
}

  // proto:  void QAbstractEventDispatcher::awake();
impl<'a> /*trait*/ QAbstractEventDispatcher_awake<()> for () {
  fn awake(self , rsthis: &mut QAbstractEventDispatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractEventDispatcher5awakeEv()};
     unsafe {_ZN24QAbstractEventDispatcher5awakeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractEventDispatcher::registerSocketNotifier(QSocketNotifier * notifier);
impl /*struct*/ QAbstractEventDispatcher {
  pub fn registerSocketNotifier<RetType, T: QAbstractEventDispatcher_registerSocketNotifier<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.registerSocketNotifier(self);
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_registerSocketNotifier<RetType> {
  fn registerSocketNotifier(self , rsthis: &mut QAbstractEventDispatcher) -> RetType;
}

  // proto:  void QAbstractEventDispatcher::registerSocketNotifier(QSocketNotifier * notifier);
impl<'a> /*trait*/ QAbstractEventDispatcher_registerSocketNotifier<()> for (QSocketNotifier) {
  fn registerSocketNotifier(self , rsthis: &mut QAbstractEventDispatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractEventDispatcher22registerSocketNotifierEP15QSocketNotifier()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QAbstractEventDispatcher22registerSocketNotifierEP15QSocketNotifier(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractEventDispatcher::flush();
impl /*struct*/ QAbstractEventDispatcher {
  pub fn flush<RetType, T: QAbstractEventDispatcher_flush<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.flush(self);
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_flush<RetType> {
  fn flush(self , rsthis: &mut QAbstractEventDispatcher) -> RetType;
}

  // proto:  void QAbstractEventDispatcher::flush();
impl<'a> /*trait*/ QAbstractEventDispatcher_flush<()> for () {
  fn flush(self , rsthis: &mut QAbstractEventDispatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractEventDispatcher5flushEv()};
     unsafe {_ZN24QAbstractEventDispatcher5flushEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractEventDispatcher::unregisterSocketNotifier(QSocketNotifier * notifier);
impl /*struct*/ QAbstractEventDispatcher {
  pub fn unregisterSocketNotifier<RetType, T: QAbstractEventDispatcher_unregisterSocketNotifier<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unregisterSocketNotifier(self);
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_unregisterSocketNotifier<RetType> {
  fn unregisterSocketNotifier(self , rsthis: &mut QAbstractEventDispatcher) -> RetType;
}

  // proto:  void QAbstractEventDispatcher::unregisterSocketNotifier(QSocketNotifier * notifier);
impl<'a> /*trait*/ QAbstractEventDispatcher_unregisterSocketNotifier<()> for (QSocketNotifier) {
  fn unregisterSocketNotifier(self , rsthis: &mut QAbstractEventDispatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractEventDispatcher24unregisterSocketNotifierEP15QSocketNotifier()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QAbstractEventDispatcher24unregisterSocketNotifierEP15QSocketNotifier(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractEventDispatcher::wakeUp();
impl /*struct*/ QAbstractEventDispatcher {
  pub fn wakeUp<RetType, T: QAbstractEventDispatcher_wakeUp<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.wakeUp(self);
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_wakeUp<RetType> {
  fn wakeUp(self , rsthis: &mut QAbstractEventDispatcher) -> RetType;
}

  // proto:  void QAbstractEventDispatcher::wakeUp();
impl<'a> /*trait*/ QAbstractEventDispatcher_wakeUp<()> for () {
  fn wakeUp(self , rsthis: &mut QAbstractEventDispatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractEventDispatcher6wakeUpEv()};
     unsafe {_ZN24QAbstractEventDispatcher6wakeUpEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractEventDispatcher::metaObject();
impl /*struct*/ QAbstractEventDispatcher {
  pub fn metaObject<RetType, T: QAbstractEventDispatcher_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QAbstractEventDispatcher) -> RetType;
}

  // proto:  const QMetaObject * QAbstractEventDispatcher::metaObject();
impl<'a> /*trait*/ QAbstractEventDispatcher_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QAbstractEventDispatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAbstractEventDispatcher10metaObjectEv()};
     unsafe {_ZNK24QAbstractEventDispatcher10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAbstractEventDispatcher::unregisterTimers(QObject * object);
impl /*struct*/ QAbstractEventDispatcher {
  pub fn unregisterTimers<RetType, T: QAbstractEventDispatcher_unregisterTimers<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unregisterTimers(self);
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_unregisterTimers<RetType> {
  fn unregisterTimers(self , rsthis: &mut QAbstractEventDispatcher) -> RetType;
}

  // proto:  bool QAbstractEventDispatcher::unregisterTimers(QObject * object);
impl<'a> /*trait*/ QAbstractEventDispatcher_unregisterTimers<i8> for (QObject) {
  fn unregisterTimers(self , rsthis: &mut QAbstractEventDispatcher) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractEventDispatcher16unregisterTimersEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN24QAbstractEventDispatcher16unregisterTimersEP7QObject(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QAbstractEventDispatcher::remainingTime(int timerId);
impl /*struct*/ QAbstractEventDispatcher {
  pub fn remainingTime<RetType, T: QAbstractEventDispatcher_remainingTime<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.remainingTime(self);
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_remainingTime<RetType> {
  fn remainingTime(self , rsthis: &mut QAbstractEventDispatcher) -> RetType;
}

  // proto:  int QAbstractEventDispatcher::remainingTime(int timerId);
impl<'a> /*trait*/ QAbstractEventDispatcher_remainingTime<i32> for (i32) {
  fn remainingTime(self , rsthis: &mut QAbstractEventDispatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractEventDispatcher13remainingTimeEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN24QAbstractEventDispatcher13remainingTimeEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAbstractEventDispatcher::startingUp();
impl /*struct*/ QAbstractEventDispatcher {
  pub fn startingUp<RetType, T: QAbstractEventDispatcher_startingUp<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.startingUp(self);
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_startingUp<RetType> {
  fn startingUp(self , rsthis: &mut QAbstractEventDispatcher) -> RetType;
}

  // proto:  void QAbstractEventDispatcher::startingUp();
impl<'a> /*trait*/ QAbstractEventDispatcher_startingUp<()> for () {
  fn startingUp(self , rsthis: &mut QAbstractEventDispatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractEventDispatcher10startingUpEv()};
     unsafe {_ZN24QAbstractEventDispatcher10startingUpEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractEventDispatcher::closingDown();
impl /*struct*/ QAbstractEventDispatcher {
  pub fn closingDown<RetType, T: QAbstractEventDispatcher_closingDown<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.closingDown(self);
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_closingDown<RetType> {
  fn closingDown(self , rsthis: &mut QAbstractEventDispatcher) -> RetType;
}

  // proto:  void QAbstractEventDispatcher::closingDown();
impl<'a> /*trait*/ QAbstractEventDispatcher_closingDown<()> for () {
  fn closingDown(self , rsthis: &mut QAbstractEventDispatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractEventDispatcher11closingDownEv()};
     unsafe {_ZN24QAbstractEventDispatcher11closingDownEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAbstractEventDispatcher::unregisterTimer(int timerId);
impl /*struct*/ QAbstractEventDispatcher {
  pub fn unregisterTimer<RetType, T: QAbstractEventDispatcher_unregisterTimer<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unregisterTimer(self);
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_unregisterTimer<RetType> {
  fn unregisterTimer(self , rsthis: &mut QAbstractEventDispatcher) -> RetType;
}

  // proto:  bool QAbstractEventDispatcher::unregisterTimer(int timerId);
impl<'a> /*trait*/ QAbstractEventDispatcher_unregisterTimer<i8> for (i32) {
  fn unregisterTimer(self , rsthis: &mut QAbstractEventDispatcher) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractEventDispatcher15unregisterTimerEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN24QAbstractEventDispatcher15unregisterTimerEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

